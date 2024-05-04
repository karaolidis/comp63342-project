class A {
    public static void f() {
    }
}

class B extends A {
    public static void f() {
        assert false;
    }
}

class virtual2 {
    public static void test(A b) {
        b.f(); // this really calls B.f, not A.f
    }
}
