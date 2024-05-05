public class multinewarray {
  public static void test(int x, int y) {
    int[][][] some_a = new int[4][3][2];

    assert some_a.length == 4;
    assert some_a[0].length == 3;
    assert some_a[0][0].length == 2;

    int[][] int_array = new int[x][y];

    for (int i = 0; i < x; ++i) {
      for (int j = 0; j < y; ++j) {
        int_array[i][j] = i + j;
      }
    }

    assert int_array[2][4] == 6;
  }
}
