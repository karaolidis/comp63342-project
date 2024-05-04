class A extends RuntimeException {
}

class B extends A {
}

public class exceptions18 {
  private static void foo() throws A {
    A a = new A();
    throw a;
  }

  public static void test() {
    try {
      foo();
    } catch (B e) {
      assert false;
    } catch (A e) {
      // expected here
    }
  }
}
