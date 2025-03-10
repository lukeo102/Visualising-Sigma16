// An array of exercises: [(name, code)]

pub const EXERCISES: [(&str, &str); 7] = [
    (
        "Lea, Load, Store Showcase",
        "
     lea   R1,1[R0]
     lea   R2,variables[R0]
     lea   R3,variables[R1]

     load  R4,variables[R0]
     load  R5,0[R2]
     load  R6,variables[R1]

     store R6,variables[R0]
     store R4,variables[R1]

     sub   R2,R2,R1
     store R3,0[R2] 

     trap  R0,R0,R0

          data 0
variables data 5
          data 10

",
    ),
    (
        "Square value in memory",
        "; A simple program to square a value stored
; in memory, then store it in the next
; memory location

      lea R1,1[R0]       ; R1 := 1
      load R2,value[R0]  ; R2 := value
      mul R3,R2,R2       ; R3 := value * value
      store R3,value[R1] ; value + 1 := R3
      trap R0,R0,R0      ; hault

value data 10

",
    ),
    (
        "Array Sum Errors",
        "; Setup i
     lee    R1,0[R0]      ; R1 := i := 0
     lea    R2,1[R0]      ; R2 := 1

; Setup len(x)
     lea    R5,n[R0]      ; R5 := &x
     lea    R6,x[R0]      ; R6 := &n
     sub    R3,R6,R5      ; R3 := len(x) := &n - &x

; Add the array
loop load   R4,x[R1]      ; R4 := x[i]
     add    R5,R5,R3      ; R5 := R5 + R4 := total + x[i]
     cmp    R1,R3
     jumpge done[R0]      ; if i >= len(x), goto done 
     add    R1,R1,R2      ; R1 := R1 + R2 := R1 + 1
     jump   l00p[R0]      ; goto loop

; Loop finished
done store  R5,result[R0] ; result := R5
     trap   R0,R0,R0      ; terminate

; Array
x      data    10
       data    4
       data    23
       data    9
       data    17
n      data    15

result data    0
",
    ),
    (
        "Array Sum Solution",
        "; Setup i
     lea    R1,0[R0]      ; R1 := i := 0
     lea    R2,1[R0]      ; R2 := 1

; Setup len(x)
     lea    R5,x[R0]      ; R5 := &x
     lea    R6,n[R0]      ; R6 := &n
     sub    R3,R6,R5      ; R3 := len(x) := &n - &x

; Add the array
loop load   R4,x[R1]      ; R4 := x[i]
     add    R5,R5,R4      ; R5 := R5 + R4 := total + x[i]
     cmp    R1,R3
     jumpge done[R0]      ; if i >= len(x), goto done 
     add    R1,R1,R2      ; R1 := R1 + R2 := R1 + 1
     jump   loop[R0]      ; goto loop

; Loop finished
done store  R5,result[R0] ; result := R5
     trap   R0,R0,R0      ; terminate

; Array
x      data    10
       data    4
       data    23
       data    9
       data    17
n      data    15

result data    0",
    ),
    (
        "Pointers Pt. 1",
        "; Squares numbers in an array using displacement

; for i := 0 to n-1
;    x[i] := x[i] * x[i]

; i := 0
; while i < n do
;    x[i] := x[i] * x[i]
;    i : = i + 1

;       i := 0
;loop   if not(i < n) goto end
;       x[i] := x[i] * x[i]
;       i := i + 1
;       goto loop
;end

; R1 := i
; R2 := n
; R3 := x[i]
; R4 := 1


         add      R1,R0,R0          ; R1 := 0 = i
         load     R2,n[R0]          ; R2 := n
         lea      R4,1[R0]          ; R4 := 1
loop     cmp      R1,R2             ; compare i and n
         jumpge   end[R0]           ; if i >= n goto end
         load     R3,x[R1]          ; R3 := x[i]
         mul      R3,R3,R3          ; R3 := x[i] * x[i]
         store    R3,x[R1]          ; x[i] := x[i] * x[i]
         add      R1,R1,R4          ; i := i + 1
         jump     loop[R0]          ; goto loop
end      trap     R0,R0,R0          ; terminate
x    data   7
     data   5
     data   2
     data   4
     data   3
n    data   5",
    ),
    (
        "Pointers Pt. 2",
        "; Squares values in an array using pointer arithmetic

; p := &x[0]
; q := p + n
; while p < q do
;   *p := *p * *p
;    p : = p + 1

;       p := &x[0]
;       q := p + n
;start  if not(p < q) goto done
;      *p := *p * *p
;       p : = p + 1
;       goto start
;done

; R1 := p
; R2 := n
; R3 := x[i]
; R4 := q


         lea      R1,x[R0]          ; R1 := &x[0] = p
         load     R2,n[R0]          ; R2 := n
         add      R4,R1,R2          ; R4 := p + n = q
start    cmp      R1,R4             ; compare p and q
         jumpge   done[R0]          ; if i >= n goto done
         load     R3,0[R1]          ; R3 := *p (x[i])
         mul      R3,R3,R3          ; R3 := *p * *p (x[i] * x[i])
         store    R3,0[R1]          ; *p := *p * *p (x[i] := x[i] * x[i])
         lea      R1,1[R1]          ; p : = p + 1
         jump     start[R0]         ; goto start
done     trap     R0,R0,R0          ; terminate
x    data   7
     data   5
     data   2
     data   4
     data   3
n    data   5",
    ),
    (
        "Pointers Pt. 3",
        "; Squares values in two arrays using pointer arithmetic and one square function

; p := &array[0]
; q := p + n
; while p < q do
;   *p := *p * *p
;    p : = p + 1

;       p := &array[0]
;       q := p + n
;start  if not(p < q) goto done
;      *p := *p * *p
;       p : = p + 1
;       goto start
;done

; R1 := p
; R2 := n
; R3 := array[i]
; R4 := q
; R9 := function return address


         jump     first[R0]         ; goto first

; Sqauraing function
function cmp      R1,R4             ; compare p and q
         jumpgt   $0000[R9]         ; if i > n goto return address
         load     R3,0[R1]          ; R3 := *p (array[i])
         mul      R3,R3,R3          ; R3 := *p * *p (array[i] * array[i])
         store    R3,0[R1]          ; *p := *p * *p (array[i] := array[i] * array[i])
         lea      R1,1[R1]          ; p  := p + 1
         jump     function[R0]      ; goto start

; Square contents of array x
first    lea      R1,x[R0]          ; R1 := &x[0] = p
         lea      R4,nx[R0]         ; R2 := &nx = q
         lea      R9,second[R0]     ; R9 := return address := &second
         jump     function[R0]      ; goto function

; Square contents of array y
second   lea      R1,y[R0]          ; R1 := &y[0] = p
         lea      R4,ny[R0]         ; R2 := &ny = q
         lea      R9,done[R0]       ; R9 := return address := &done
         jump     function[R0]      ; goto function

done     trap     R0,R0,R0          ;terminate

; Array x
x    data   7
     data   5
     data   2
     data   4
     data   3
nx   data   5

; Array y
y    data   8
     data   10
     data   2
     data   4
     data   21
     data   13
     data   17
ny   data   7
",
    ),
];
