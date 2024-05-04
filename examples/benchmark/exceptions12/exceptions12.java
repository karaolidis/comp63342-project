class A extends RuntimeException {
}

class B extends A {
}

class C extends B {
}

class F {
  void foo() {
    B b = new B();
    throw b;
  }
}

public class exceptions12 {
  public static void test() {
    F f = new F();
    f.foo();
  }
}
