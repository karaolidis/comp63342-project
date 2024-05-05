public class ArrayIndexOutOfBoundsException2 {

  public static void test(int size) {
    if (size < 0) return;
    try {
      int[] a = new int[4];
      int i = a[size];
    } catch (ArrayIndexOutOfBoundsException exc) {
      assert false;
    }
  }
}
