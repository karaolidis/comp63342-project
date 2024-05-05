public class ArithmeticException1 {
  public static void test(int nondetInt) {
    try {
      int i = nondetInt;
      int j = 10 / i;
    } catch (ArithmeticException exc) {
      assert false;
    }
  }
}
