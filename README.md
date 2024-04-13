# Szyfr Cezara i afiniczny w Rust oraz atak bruteforce na ich implementacje
Celem laboratorium była implementacja szyfru Cezara oraz szyfru afinicznego w języku Rust
oraz implementacja ataku brute-force na szyfrogram. 
Zimplementowany przez nas program został podzielony na moduły dla zachowania przejrzystości. 
## Moduły
### main.rs
Główny moduł programu odpowiedzialny za interakcję z użytkownikiem poprzez wiersz poleceń. 
Wykorzystuje bibliotekę clap do parsowania argumentów oraz odpowiada za ich walidację. 
Na podstawie przekazanych argumentów uruchamiane są odpowiednie funkcje z innych modułów.
### brute_force.rs
Moduł zawierający logikę ataku brute_force. Główne funkcje to:
#### count_chi_for_encrypted_text
Funkcja oblicza odchylenie statystyczne dla szyfrogramu. Używa funkcji count_ngrams stworzonej w poprzednim laboratorium.
następnie oblicza częstotliwość występowania poszczególnych monogramów w tekście. Następnie używając implementacji wzoru 
podanego w instrukcji laboratoryjnej oblicza i zwraca odchylenie statystyczne.
#### calculate_chi_squared
Funkcja porównuje odchylenia statystyczne szyfrogramu i referencyjne wartości obliczone na podstawie odpowienio długiego tekstu w języku angiejskim.
#### brute_force_caesar
Podstawą tej funkcji jest pętla for, w której testowane są wszystkie litery alfabetu łacińskiego. 
Dla rezultatu  obliczane jest odchylenie statystyczne. Wynik najbliższy pierwowzorowi z laboratorium 1,
najprawdopodobniej jest rozwiązaniem. 
#### brute_force_affine
W tej funkcji znajduje się tablica współczynników a, których można użyć w szyfrze afinicznym. 
Jest to zbiór liczb, które są względnie pierwsze z 26, ilością liter w łacińskim alfabecie. 
Jeśli a nie jest względnie pierwsza z 26, nie da się obliczyć jej odwrotności, wobec czego nie da się odczytać szyfrogramu.
W zagnieżdżonej pętli for iterujemy po wszystkich możliwych wartościach a i b. Następnie analogicznie do brute_force_caesat
odchylenie statystyczne jest porównywane z wartościami referencyjnymi. 
### affine.rs
W tym module znajdują się dwie główne publiczne odpowiedzialne za szyfrowanie i odszyfrowywanie wiadomości.
Najpierw wszystkie litery tekstu wejściowego konwertowane są na wielkie litery. Następnie odfiltrowywane są 
wszystkie znaki, które nie są literami alfabetu. Następnie każda litera jest przetwarzana w następujący sposób:
- litera konwertowana jest na wartość liczbową (A-0, B-1...)
- właściwe szyfrowanie, zgodnie z matematyczny wzorem szyfru afinicznego.
- dodatkowo zastosowano korektę ujemnych wartości poprzez dodanie M i dodatkową operację modulo.
Funkcja decrypt działa analogicznie. Funkcja inverted_mod jest funkcją pomocniczą obliczającą odwrotność modulo a.
### caesar.rs
Ten moduł zawiera tylko jedną funkcję - cipher. Jako, że w szyfrze Cezara, żeby odszyfrować wiadomość wystarczy użyć wartości ujemnej klucza.
Najpierw wszystkie litery wejściowe konwertowane są na wielkie litery oraz oczyszczane ze znaków specjalnych. 
Następnie litery konwertowane są na ich wartości liczbowe. Wartość key jest normalizowana za pomocą metody rem-euclid, co gwarantuje, że liczba będzie w zakresie 0-25.
Następnie wykonywane jest "przesunięcie" - wartość klucza dodawana jest do wartości liczbowej litery oraz obliczane jest modulo 26 tej wartości,
aby upewnić się, że liczba znaduje sie w zakresie 0-25.
## Komendy
### Szyfrowanie szyfrem Cezara
`cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -k 3 -e --caesar `
### Deszyfrowanie szyfru Cezara
`cargo run -- -i src/resource/encrypted.txt -o src/resource/decrypted.txt -k 3 -d --caesar`
### Szyfrowanie szyfrem afinicznym
`cargo run -- -i src/resource/plain.txt -o src/resource/encrypted.txt -e --affine -a 3 -b 5 `
### Deszyfrowanie szyfru afinicznego
`cargo run -- -i src/resource/encrypted.txt -o src/resource/decrypted.txt -d --affine -a 3 -b 5`
### Brute force dla szyfru Cezara
`cargo run -- -i src/resource/encrypted.txt -o src/resource/brute_decrypted.txt --brute --caesar`
### Brute force dla szyfru afinicznego
`cargo run -- -i src/resource/encrypted.txt -o src/resource/brute_decrypted.txt --brute --affine`