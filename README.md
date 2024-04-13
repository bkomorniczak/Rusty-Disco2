`cargo run -- -i src/resource/encrypted.txt --brute `

`cargo run -- -i src/resource/encrypted.txt -o src/resource/decrypted.txt -k 3 -d --caesar   
`
`cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k 3 -e --caesar `
`cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -e --affine -a 2 -b 5 `

