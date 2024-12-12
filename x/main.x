let n = 200; // 你想计算的斐波那契数列的项数

let fibPrevPrev = 0; // 斐波那契数列的前前项
let fibPrev = 1;    // 斐波那契数列的前一项
let fibCurrent=0; 
for (let i1 = 2; i1 < n; i1=i1+1) {
for (let i2 = 2; i2 < n; i2=i2+1) {
        print i1+i2;   
}           
}