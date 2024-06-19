use std::io; //標準ライブラリの取り入れ。Cのincludeのような物。
use std::cmp::Ordering; //新たなスコープの取り入れ
/*OrderingはenumでLess,Greater,Equalという列挙子を持っている。（比較より、小さいか大きいか同じかを返す） */
use rand::Rng; /*Rngトレイトはメソッド定義。 OSからシード値を得ており、スレッドに固定されている*/
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //1..101で１から１００までとしている。上限は含めないので注意。

    println!("The secert number is: {}", secret_number);
    
loop{//loop while文と同じような感じ？　
    println!("Please input your guess.");
    
    let mut guess = String::new();
     /*ユーザーからのINPUT情報を格納する変数を作っている。
     mutは何か。Mutable(可変)の略。基本的にRUSTではできない、可変的な変数に変化。
    INPUTされているため、mutは必ず必要になる。*/

    io::stdin() //std::io::stdinとここで記入することで、ioライブラリを最初にいれてなくても、使えるようになる。 
    .read_line(&mut guess)
    /*read_lineメソッドを呼び出している。引数として&mut guessをとることで、read_lineの機能は入力したデーターを文字列に格納する。
    「&」は引数が参照であることを指す。Cの参照と同じように参照にすることで、データをメモリに何度もコピーをすることがなくなる。*/
    
    .expect("Failed to read line");        
 /*Result形で失敗を扱う。先ほどの行と合わせて、io::stdin().expect("Failed to read line"); 
長くなるので、分割している。.expectは何をしているのか、read_lineメソッドはio::Resultを返す。
Resultは列挙型になっており、列挙子はOkかErrを返す。つまり処理に対してOKならば成功。失敗ならば、Errが課程や理由を記載して返す。
Result型では、他の型同様にメソッドが定義されており、expectメソッドがそうなる。io::ResultインスタンスがErrの場合プログラムをクラッシュさせて、
メッセージを返す。OKの場合、ok列挙子が持つ戻り値を取り出し、その値を返す。今回の場合はユーザー入力のバイト数になる。*/
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num, //.expect("Please type a number!");から match式に切り替えて、Result型がOkならば、num（数値）を返す。
    Err(_) => continue,//エラーならば、continueで再度戻るような式を返す。
};
     
/*let guess: u32 = guess.trim().parse().expect("Please type a number!");はString型を実数型に変換している動作をしている。guess変数は
すでにあるが、ここでは変数をシャドーイングしている。シャドーイングは他の言語ではエラーになるが、変数名を再利用させてくれている。guess.trim()の中にある
guessは元のguessを指す。trimメソッドは文字列の先頭と末尾の空白を削除する。こうすることで、数値のデータのみを扱えるようになる。parseメソッドは
文字列を解析して、何らかの数値にしている。Cにおける「atoi」のようなものだと思えばわかりやすい。guess のあとに（：）を入れることで、
これが何を指しているのかを正確な数値の型を示すことができます。今回の場合はu32型が指し示られている。parseメソッドはAや絵文字、記号などが
入っている場合はエラーになり、Result型を返す */

    println!("You guessed: {}",guess);//{}はプレースホルダーになる。変数の格納先のようなものだと思えばよい。

    match guess.cmp(&secret_number){
        Ordering::Less =>println!("Too small!"),
        Ordering::Greater =>println!("Too big!"),
        Ordering::Equal =>{
            println!("YOU WIN!");
            break;  //break文でloopを抜けるようになる。
        }
    }
 }
}

/*rustには乱数機能はないために、randクレートを使う。現在のこのプロジェクトはバイナリクレートのため
ライブラリクレートと連携する必要がある。Cargo.tomlを編集してrandクレートを依存関係に含める。Cargo.tomlの[dependecies]セクション
ヘッダの下に含める。cargoは自動的に新しいバージョンを使用しますが、新バージョンを利用するためにはcargo updateコマンドを利用する
することでも、更新可能。 */