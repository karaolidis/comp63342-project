class A extends RuntimeException {
}

class B extends A {
}

class C extends B {
}

public class exceptions16 {
  public static void test(int x) {
    x++; // increment x

    throw new B();
  }
}
