public class ArrayIndexOutOfBoundsException1 {

  public static void test(int size) {
    if (size < 0) return;
    try {
      int[] a = new int[4];
      a[size] = 0;
    } catch (ArrayIndexOutOfBoundsException exc) {
      assert false;
    }
  }
}
