class A extends RuntimeException {
}

class B extends A {
}

class C extends B {
}

public class exceptions8 {
  static void foo() throws B {
    B b = new B();
    throw b;
  }

  public static void test() {
    A a = new A();
    foo();
    throw a;
  }
}
