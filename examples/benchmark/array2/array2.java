public class array2 {

  public static void test(int unknown) {
    int[] arr;
    if (unknown > 0)
      arr = new int[1];
    else
      arr = new int[0];

    if (unknown > 0)
      arr[0] = 1;

    if (unknown > 0)
      assert arr.length == 1 && arr[0] == 1;
    else
      assert arr.length == 0;
  }
}
