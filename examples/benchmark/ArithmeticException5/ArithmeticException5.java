public class ArithmeticException5 {
  
  public static void test(double i) {
    try {
      double j = 10 / i;
    } catch (ArithmeticException exc) {
      assert false;
    }
  }
}
