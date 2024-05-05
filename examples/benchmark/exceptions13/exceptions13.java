class A extends RuntimeException {}

class B extends A {}

class C extends B {}

class F {
  public static void foo() throws B {
    try {
      B b = new B();
      throw b;
    } catch (B exc) {
      throw exc;
    }
  }
}

public class exceptions13 {

  public static void test() {
    try {
      F.foo();
    } catch (B exc) {
      assert false;
    }
  }
}
