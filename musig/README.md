# MuSig

## Params

- p: order of scalar field
- g: basepoint of elliptic curve group
- H: hash function $H: \{0,1\}^* \rightarrow \mathbb Z_q$
- m: message to be signed

## Key Generation

- private key: $x_1, .., x_i \in \mathbb Z_q$
- public key: $y_1, .., y_i = x_i * g$
