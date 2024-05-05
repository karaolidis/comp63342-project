public class ArithmeticException6 {
  public static void test(int denom) {
    try {
      int j = 10 / denom;
    } catch (ArithmeticException exc) {
      assert false;
    }
  }
}
