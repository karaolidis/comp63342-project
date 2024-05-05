public class ArrayIndexOutOfBoundsException3 {
  public static void test(int index) {
    try {
      int[] a = new int[4];
      a[index] = 0;
    } catch (Exception exc) {
      assert false;
    }
  }
}
