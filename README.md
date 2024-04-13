`cargo run -- -i src/resource/encrypted.txt -o src/resource/brute_decrypted.txt --brute --caesar`

`cargo run -- -i src/resource/encrypted.txt -o src/resource/brute_decrypted.txt --brute --affine`

`cargo run -- -i src/resource/encrypted.txt -o src/resource/decrypted.txt -k 3 -d --caesar   
`
`cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k 3 -e --caesar `

`cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -e --affine -a 3 -b 5 `

`cargo run -- -i src/resource/encrypted.txt -o src/resource/decrypted.txt -d --affine -a 3 -b 5 `

