public class Classes0 {
    class A {
        B b;
    }

    class B {
        int x;
    }

    public static void test(A a) {
        assert a.b.x == 0;
    }
}
