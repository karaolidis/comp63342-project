public class NegativeArraySizeException2 {
  public static void test() {
    try {
      int a[] = new int[-1];
    } catch (Exception exc) {
      assert false;
    }
  }
}
