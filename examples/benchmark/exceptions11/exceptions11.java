class A extends RuntimeException {
  int i = 1;
}

class B extends A {
}

public class exceptions11 {
  static int foo(int k) {
    if (k == 0) {
      A a = new A();
      throw a;
    } else {
      A b = new A();
      throw b;
    }
  }

  public static void test(int k) {
    try {
      A a = new A();
      foo(k);
    } catch (A exc) {
      assert exc.i == 1;
    }
  }
}
