class A {
  int i;
}

public class NullPointerException4 {
  public void test(A a) {
    a.i = 0;  // purposefully setting this without checking if `a` is null to simulate the issue
  }
}
