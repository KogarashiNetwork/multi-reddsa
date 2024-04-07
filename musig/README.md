# MuSig

## Params

- p: order of scalar field
- g: basepoint of elliptic curve group
- H: hash function $H: \{0,1\}^* \rightarrow \mathbb Z_q$
- m: message to be signed

## Signature Aggregation

### Setup

- Alice private key: $x_A \in \mathbb Z_q$
- Alice public key: $y_A = x * g$
- Alice randomness $k_A \in \mathbb Z_q$
- Bob private key: $x_B \in \mathbb Z_q$
- Bob public key: $y_B = x * g$
- Bob randomness $k_B \in \mathbb Z_q$
- Aggregated public key: $y = y_A + y_B$
- Aggregated randomness: $R = k_A * g + k_B * g$

### Sign

- Alice computes $s_1 = k_1 - cx_1$
- Bob computes $s_2 = k_2 - cx_2$
- let $s = s_1 + s_2$
- let $(R, s)$ signature

### Verify

- let $r_v = s * g + e * y$
- let $e_v = H(r_v || M)$

if $e_v = e$, the signature is valid.
