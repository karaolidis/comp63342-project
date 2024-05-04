/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/cbmc-java/instanceof2
 * The benchmark was taken from the repo: 24 January 2018
 */
class instanceof2 {
  public static void test() {
    assert int.class instanceof Object;
  }
}
