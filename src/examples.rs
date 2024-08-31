pub fn get_example(name: &str) -> &'static str {
    match name {
        "easy" => "
-4- --- --5
568 -1- 4--
1-7 -54 -6-

--- --8 ---
73- 162 -98
685 47- -3-

4-- 8-- 25-
--- 7-6 3--
-26 3-5 --1
    ",
    "hard" => "
8-- --- ---
--3 6-- ---
-7- -9- 2--

-5- --7 ---
--- -45 7--
--- 1-- -3-

--1 --- -68
--8 5-- -1-
-9- --- 4--
    ",

    "empty" => "
--- --- ---
--- --- ---
--- --- ---

--- --- ---
--- --- ---
--- --- ---

--- --- ---
--- --- ---
--- --- ---
    ",

    "extreme" => "
91---35-2
---4--3--
-8-------
----7--6-
--5------
23---4--1
59-6---1-
--4-----5
--8--9---",
_ => panic!("unknown example!"),
}
}

