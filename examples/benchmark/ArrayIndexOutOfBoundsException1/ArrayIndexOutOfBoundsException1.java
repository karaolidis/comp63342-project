public class ArrayIndexOutOfBoundsException1 {
  public void test(int size) {
    if (size < 0)
      return;
    int[] a = new int[4];
    if (size >= 4) {
      assert false;
    } else {
      a[size] = 0;
    }
  }
}
