/// # Example: Some signed KEX
/// This protocol has no replay protection!
///
use oqs::*;
fn main() -> Result<()> {
    let sigalg = sig::Sig::new(sig::Algorithm::Dilithium2)?;
    let kemalg = kem::Kem::new(kem::Algorithm::Kyber512)?;
    // A's long-term secrets
    let (a_sig_pk, a_sig_sk) = sigalg.keypair()?;
    // B's long-term secrets
    let (b_sig_pk, b_sig_sk) = sigalg.keypair()?;

    // assumption: A has (a_sig_sk, a_sig_pk, b_sig_pk)
    // assumption: B has (b_sig_sk, b_sig_pk, a_sig_pk)

    // A -> B: kem_pk, signature
    let (kem_pk, kem_sk) = kemalg.keypair()?;
    let signature = sigalg.sign(kem_pk.as_ref(), &a_sig_sk)?;

    // B -> A: kem_ct, signature
    sigalg.verify(kem_pk.as_ref(), &signature, &a_sig_pk)?;
    let (kem_ct, b_kem_ss) = kemalg.encapsulate(&kem_pk)?;
    let signature = sigalg.sign(kem_ct.as_ref(), &b_sig_sk)?;

    // A verifies, decapsulates, now both have kem_ss
    sigalg.verify(kem_ct.as_ref(), &signature, &b_sig_pk)?;
    let a_kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct)?;
    assert_eq!(a_kem_ss, b_kem_ss);

    Ok(())
}