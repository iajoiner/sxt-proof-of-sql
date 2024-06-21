use ark_std::test_rng;
use proof_of_sql::{
    base::{
        commitment::{CommitmentEvaluationProof, QueryCommitments, QueryCommitmentsExt},
        database::{owned_table_utility::{owned_table, bigint}, OwnedTable, OwnedTableTestAccessor, TestAccessor},
    },
    proof_primitive::dory::{
        DoryCommitment, DoryEvaluationProof, DoryScalar, DoryProverPublicSetup, DoryVerifierPublicSetup,
    },
    sql::{
        parse::QueryExpr,
        proof::{ProofExpr, ProvableQueryResult, QueryProof},
    },
};

fn run_verification<CP: CommitmentEvaluationProof>(
    query: &str,
    default_schema: &str,
    query_commitments: &QueryCommitments<CP::Commitment>,
    proof: &QueryProof<CP>,
    serialized_result: &ProvableQueryResult,
    verifier_setup: &CP::VerifierPublicSetup,
) -> Option<OwnedTable<CP::Scalar>> {
    let query_expr = QueryExpr::try_new(
        query.parse().ok()?,
        default_schema.parse().ok()?,
        query_commitments,
    )
    .ok()?;
    Some(
        proof
            .verify(
                query_expr.proof_expr(),
                query_commitments,
                serialized_result,
                verifier_setup,
            )
            .ok()?
            .table,
    )
}

/// This method verifies a proof for a given query and returns the result if the proof is valid.
/// This method will return `None` if the results cannot be verified.
///
/// The inputs are:
///     - `query`: The SQL query to verify.
///     - `default_schema`: The default schema to use for the query.
///     - `query_commitments`: The commitments to the columns of data used in the query.
///     - `proof`: The proof of the query result.
///     - `serialized_result`: The serialized result of the query.
///     - `verifier_setup`: The public setup for the verifier.
pub fn run_dory_verification(
    query: &str,
    default_schema: &str,
    query_commitments: &QueryCommitments<DoryCommitment>,
    proof: &QueryProof<DoryEvaluationProof>,
    serialized_result: &ProvableQueryResult,
    verifier_setup: &DoryVerifierPublicSetup,
) -> Option<OwnedTable<DoryScalar>> {
    run_verification(
        query, // parameter (javascript URL would be another parameter for smart contract)
        default_schema, // hardcode
        query_commitments, // from API
        proof, // from API
        serialized_result, // from API
        verifier_setup, // from a file (IPFS binary blob) could be hardcoded (doesn't change)
    )
}

fn main() {
    let dory_prover_setup = DoryProverPublicSetup::rand(4, 3, &mut test_rng());
    let dory_verifier_setup: DoryVerifierPublicSetup = (&dory_prover_setup).into();

    let mut accessor = OwnedTableTestAccessor::<DoryEvaluationProof>::new_empty_with_setup(
        dory_prover_setup.clone(),
    );
    accessor.add_table(
        "sxt.table".parse().unwrap(),
        owned_table([bigint("a", [1i64, 2, 3]), bigint("b", [1i64, 0, 1])]),
        0,
    );
    let query = QueryExpr::try_new(
        "SELECT * FROM table WHERE b = 1".parse().unwrap(),
        "sxt".parse().unwrap(),
        &accessor,
    )
    .unwrap();
    let (proof, serialized_result) =
        QueryProof::<DoryEvaluationProof>::new(query.proof_expr(), &accessor, &dory_prover_setup);
    let query_commitments = QueryCommitments::from_accessor_with_max_bounds(
        query.proof_expr().get_column_references(),
        &accessor,
    );

    let query = "SELECT * FROM table WHERE b = 1".to_string();
    let default_schema = "sxt".to_string();

    let owned_table_result_opt = run_dory_verification(
        query.as_str(),
        default_schema.as_str(),
        &query_commitments,
        &proof,
        &serialized_result,
        &dory_verifier_setup,
    );

    match owned_table_result_opt {
        Some(owned_table_result) => {
            let expected_result = owned_table([bigint("a", [1i64, 3]), bigint("b", [1i64, 1])]);
            println!("Verification OK, result match: {}", owned_table_result == expected_result);
        }
        None => println!("VERIFICATION FAILED"),
    };
}
