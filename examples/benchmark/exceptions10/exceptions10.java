class A extends RuntimeException {}

class B extends A {}

class C extends B {}

public class exceptions10 {
  public static void foo() {
    try {
      A b = new A();
      throw b;
    } catch (A exc) {
      assert false;
    }
  }

  public static void test() {
    try {
      A a = new A();
      foo();
    } catch (B exc) {
      assert false;
    }
  }
}
