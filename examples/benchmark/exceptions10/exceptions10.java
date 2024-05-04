class A extends RuntimeException {
}

class B extends A {
}

class C extends B {
}

public class exceptions10 {
  static void foo() {
    A b = new A();
    throw b;
  }

  public static void test() {
    A a = new A();
    foo();
  }
}
