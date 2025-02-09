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

    /**
    * タグ無しのコメント
    * @ubiquitous ubiquitous langage2
    * @context context2
    * @description description2
    */
    class Sample2 {
        public $name;
        public $age;
        
        public function __construct($name, $age) {
            $this->name = $name;
            $this->age = $age;
        }
        
        public function sayHello2() {
            echo "Hello, " . $this->name . "!";
        }
    }
?>