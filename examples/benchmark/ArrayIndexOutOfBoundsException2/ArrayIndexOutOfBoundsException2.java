public class ArrayIndexOutOfBoundsException2 {
  public static void test(int size) {
    if (size < 0)
      return;

    int[] a = new int[4];
    assert size >= a.length : "Index out of bounds";
  }
}
