class A {
    public void f() {}
}

class B extends A {
    public void f() {
        assert false;
    }
}

class virtual2 {
    public static void test(A b) {
        b.f(); // this really calls B.f, not A.f
    }
}
