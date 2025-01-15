<?php
    // 1行目のコメント
    echo "Hello, World!";
    // 2行目のコメント
    /* ブロックコメント */
    
    /**
    * タグ無しのコメント
    * @ubiquitous ubiquitous langage
    * @context context
    * @description description
    */
    class Sample {
        public $name;
        public $age;
        
        public function __construct($name, $age) {
            $this->name = $name;
            $this->age = $age;
        }
        
        public function sayHello() {
            echo "Hello, " . $this->name . "!";
        }
    }
?>