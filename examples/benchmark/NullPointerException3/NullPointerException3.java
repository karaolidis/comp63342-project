class A {
  int i;
}

public class NullPointerException3 {
  public void test(A a) {
    if (a == null) {
      throw new NullPointerException();
    }
    int i = a.i; // Use the object normally without a try-catch
  }
}
