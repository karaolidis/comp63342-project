class A extends RuntimeException {}

class B extends A {}

class C extends B {}

class F {
  public static void foo() {
    try {
      B b = new B();
      throw b;
    } catch (B exc) {
      assert false;
    }
  }
}

public class exceptions12 {
  public static void test() {
    try {
      F.foo();
    } catch (B exc) {
      assert false;
    }
  }
}
