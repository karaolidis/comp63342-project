/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/cbmc-java/instanceof8
 * The benchmark was taken from the repo: 24 January 2018
 */
public class instanceof8 {
  
  public static boolean test(Integer i) {
    if (i instanceof Integer) {
      return true;
    } else {
      return false;
    }
  }

  public static void test() {
    assert (!test(null));
    assert (test(new Integer(1)));
  }
}
