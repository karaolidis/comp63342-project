/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/cbmc-java/interface1
 * The benchmark was taken from the repo: 24 January 2018
 */
interface A {
  public static void f();
}

class B implements A {
  public static void f() {
    assert false; // should fail
  }
}

class C implements A {
  public static void f() {
  }
}

class interface1 {
  public static void test(A b, A c) {
    b.f(); // really calls B.f
  }
}
