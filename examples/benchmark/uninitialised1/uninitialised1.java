/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/cbmc-java/uninitialised1
 * The benchmark was taken from the repo: 24 January 2018
 */
class uninitialised1 {
  public static void test() {
    int i[] = new int[10];
    assert i[3] == 0;
  }
}
