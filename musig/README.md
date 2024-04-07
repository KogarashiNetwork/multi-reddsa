# MuSig
Implementation of [Simple Schnorr Multi-Signatures with Applications to Bitcoin](https://eprint.iacr.org/2018/068)

## Signature Aggregation

### Params

- p: order of scalar field
- g: basepoint of prime order elliptic curve group
- H: hash function $H: \{0,1\}^* \rightarrow \mathbb F_q$
- m: message to be signed

### Setup

**KeyGen**
- private keys: $x_1,..,x_i \in F_q$
- public keys: $X_1,...,X_i = x_1 * g,..,x_i * g \in E(F_q)$

**PublicParams**
- aggregated public key: $\overline X = \prod_{i=1}^nX^{a_i}_i$
- randomness: chooses $r_i \in F_q$, computes $R_i = g^{r_i}$ and $t_i=H_{com}(R_i)$
- aggregated randomness: $R = \prod_{i=1}^nR_i$
- aggregated challenge: $c = H_{sig}(\overline X, R, m)$

### Sign

- let $s_i = r_i + ca_ix_i$
- let $s = \sum_{i=1}^ns_i$
- let $(R, s)$ signature

### Verify

- let $a_i = H_{agg}(L, X_i)$ for $i \in {1,2,...,i}$
- let $\overline X = \prod_{i=1}^nX^{a_i}_i$
- let $c = H_{sig}(\overline X, R, m)$
- let $r_v = s * g$

if $r_v = R + \overline X^c$, the signature is valid.
