public class NegativeArraySizeException1 {
  public static void test() {
    try {
      int a[] = new int[-1];
    } catch (NegativeArraySizeException exc) {
      assert false;
    }
  }
}
