# Juggernaut  
Juggernautは時限爆弾型のタイマーの構造を，与えられた回路図とプログラムから推測し規定内の操作を行うことでタイマーを停止させる（解除する）競技です．    
作問者には回路構成力とプログラムを柔軟に書く力が，解除する側には回路図やデータシートを読む力とプログラムを素早く間違いなく解読する力が求められ，教育面での効果が期待できます．    
解除するまでの時間を競う競技型や，お互いに装置を作って解除しあってワイワイするイベント型を想定しています．    
最新版の資料は[https://github.com/Takana-Norimasa/Juggernaut](https://github.com/Takana-Norimasa/Juggernaut)にあります．  
  
  
## 競技の流れ  
1. 運営によるルールの決定と通知（1週間ほど前）  
1. giverによる作問と回路図，プログラムの提出（前日または当日）  
1. solverへの装置と資料配布（当日）  
1. スタートシグナル（競技開始）  
1. 競技中  
1. エンドシグナル（競技終了）  
1. giverの講評  
1. 結果発表  
  
  
## 競技規則  
2020 10.31 (ver 0.0.4)  
  
### A. 総則  
1. 本競技は運営，出題者，回答者の3つの立場で構成される．  
1. 装置を用意する側を**giver**（出題者），装置を解除する側を**solver**（回答者）と呼ぶ．  
1. 装置に取り付けられたタイマーが0になる前にプログラムで決められた状態になった場合**succeeded**（または解除）となる．  
1. 装置に取り付けられたタイマーが0になる，もしくはプログラムで定められた手順から外れた状態になった場合**failed**（または解除失敗）となる．  
1. 競技は運営のスタートシグナルで始まり，succeeded/failedまたはエンドシグナルで終了する．  
1. 禁止事項（後述の競技規則C項6条，D項3条）に抵触した参加書は最終結果から除外される．  
  
### B. timer  
1. 競技に使われるタイマー型の電子回路を**timer**（または単に装置）と呼ぶ．  
1. 装置は以下の3つの部分に分けられる．  
	1. 規定のマイコンを載せた**control**（制御部）  
	1. 6桁の7セグLEDや解除状態を示すLEDなど競技進行に必要な部品をを載せた**display**（表示部）  
	1. ユーザ(giver)が自由に回路を組み制御部に入力を与える**gaming**(競技部)  
1. controlには以下の機能を備えなくてはならない．  
	1. 計測と7セグメントLEDによるカウントダウン機能  
	1. サーバとの通信機能  
	1. succeeded/in-process/failedを判定する機能  
1. displayには以下の部品を備えなくてはならない．  
	1. 残り時間を表示するための4もしくは6桁の7セグメントLED  
	1. 現在の装置の状態を示す赤(failed)，黄色(in-process)，緑(succeeded)のLED  
	1. ユーザへの注意を促すブザー  
1. 運営はgamingについて予め以下のことを通知しなくてはならない．  
	1. 使用可能なブレッドボード（または基板）のサイズとのホール数  
	1. 使用可能な部品の種類  
	1. 使用可能なワイヤの本数  
  
### C. giver  
1. giverはtimer（または装置）を提供するものを指す．  
1. giverは装置を作成するにあたり上記B項の規則を守らなければならない．  
1. giverは装置に関する完全なgamingの回路図とcontrolのプログラムをsolverに提供しなくてはならない．  
1. giverは装置の解除に可能な操作を指定しなければならない．  
1. giverは装置の所定の位置に運営が定めた識別用のラベルを貼らなくてはならない．  
1. giverは装置を所定の時間の15%以内の時間で一度解除し動作を確認しなければならない．  
1. giverは以下の禁止事項を遵守しなかればならない．  
	1. 装置には必ず規定で定められた共通の表示部を備えなくてはならない  
	1. 装置に対して，事前に決められた本数のワイヤを超えて使用することはできない  
	1. 装置に対して，事前に決められた部品以外を使用することはできない  
	1. 部品や配線の確認が困難となるような妨害行為（型番や結線部を何かで隠す等）をしてはならない  
  
### D. solver  
1. solverはtimer（または装置）を解除するものを指す．  
1. solverは与えられた資料と外部の資料（インターネット上の資料を含む）をもとに装置を独力で解除しなくてはならない．  
1. solverは以下の禁止事項を遵守しなかればならない．  
	1. 装置の表示部や制御部に手を加えてはならない  
	1. 装置の競技部に規定外の操作を行ってはならない  
  
  
  
## 解説  
回路とプログラムの総合的な力をうまいこと学んだり競い合える企画はないかな？という感じで思いつきました．  
簡単に言えば，爆弾処理に技術要素を盛り込んで競技化したものです．  
giverという役割の人がtimerという装置を作り，それをsolverが解いていきます．  
  
giverは罠を仕掛ける側です．  
決められた部品の中で如何にギミックを構築するか，そして制御プログラムを如何に難しく書くかが競技の鍵です．  
性格の悪さが求められると思います．  
  
solverはひたすら与えられたデータを元に回路と制御プログラムを読み解き解除していきます．  
時間がない中ですばやく確実にtimerの仕組みを把握できるかが勝負の鍵です．  
回路図とプログラムを読む力と冷静な判断力が求められます．  
  
お互いにtimerを作って解除し合う感じのイベント型（勉強がメイン）と運営が用意したtimerを解除する競技型の2つの運用方法を想定しています．  
装置の大まかなレギュレーションは上記の競技規則の通りなのですが，イベント型ならブレッドボードの上で，競技型なら専用基板を作るのがいいと思ってます．  
回路構成は制御装置，4桁7セグメントLED，3色のカラーLEDと別途gaming（競技領域）のブレッドボードを想定していて，displayとcontrolを構築するのはそこまで大変ではありません．  
displayとconrolはレイアウトがすでに決まっているので，大きな大会なら基板に起こすのも手だと思います．構成する部品もそこまで高価ものはなく単純なのである程度予算があれば可能だと思います．（キット化も考えてます）  
制御装置はESP32の想定です．非同期処理ができる他，サーバとの通信もできるので制御が柔軟でシグナルの受け渡しやサーバ上の可視化が容易です．  
なにより安く手に入るため低コストで競技を行うことができます．  
![](./image/DSC_0869.JPG)
  
とにかくtimerにおいては見た目が重要で，題材が題材なために誤解による騒ぎには十分注意しなければなりません．
C項4条にもあるとおり，競技用の装置であるということがひと目で分かるということは非常に重要な要素だと思います．（キット化を考えているのは見た目を統一し安全を保証するためでもあります）
  
  
