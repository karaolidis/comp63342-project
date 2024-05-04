class A extends RuntimeException {
  int i;

  A() {
    i = 1;
  }
}

class B extends A {
}

class C extends B {
}

public class exceptions6 {
  public static void test() {
    try {
      int i = 0;
      throw new A();
    } catch (A exc) {
      assert exc.i == 1; // Changed false assertion from exc.i == 2 to exc.i == 1 that matches the
                         // initialized value in A's constructor
    }
  }
}
