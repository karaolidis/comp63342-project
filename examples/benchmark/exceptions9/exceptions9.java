class A extends RuntimeException {
  int i = 1;
}

class B extends A {}

public class exceptions9 {
  public static int foo(int k) {
    try {
      if (k == 0) {
        A a = new A();
        throw a;
      } else {
        A b = new A();
        throw b;
      }
    } catch (B exc) {
      assert exc.i == 1;
    }
    return 1;
  }

  public static void test(int testParameter) {
    try {
      A a = new A();
      foo(testParameter);
    } catch (A exc) {
      assert exc.i == 1;
    }
  }
}
