// Serie nome inizia con S
// File nome inizia con F
// Regex nome inizia con R

// Funzioni speciali
// CONCAT (identity, value)
// if identity not present add new value
// D64( string ) -> double
// S32( string ) -> float
// I{16,32,64}( string ) -> i{16,32,64}


// Lista file
// se non presente solo un file chiamato F1
//
### File

// Modi per dichiarare i file
// Andando a capo
// F1 - id 1
// F2 - id 2
// F3 - id 3
// FCongilio - id Coniglio
// F1-9 - id da 1 a 9

F1-2


### Regex con nomi (lexer) ###

"[A-Z]"   Rid
"[0-9]"   Rnum
"come?"     R3
"time:"     Rtime
"Value Begin" Rbegin
"Value End" Rend


### Cosa riconoscere (parser) ###

Rid Rnum {
    CONCAT( F?Rid, D64(Rnum) )
}

Rbegin (Rnum)* Rend {
    CONCAT( F?Rid, D64(Rnum) )*
}

Rtime 


### Cosa Stampare ###











