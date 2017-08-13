package com.github.sadikovi;

public class Main {
  public static void main(String[] arr) {
    System.out.println("== Start ==");

    System.out.println("\n== Matrix 1 ==\n");

    // Show matrix 1 that is constructed from provided array
    DoubleMatrix m1 = DoubleMatrix.anew(2, 2, new double[]{1.0, 2.0, 3.0, 4.0});
    System.out.println("Matrix: " + m1);
    System.out.println("Rows: " + m1.rows());
    System.out.println("Cols: " + m1.cols());
    m1.show();
    m1.dealloc();
    System.out.println("Matrix: " + m1);

    System.out.println("\n== Matrix 2 ==\n");

    // Show matrix 2 that is randomly generated
    DoubleMatrix m2 = DoubleMatrix.rand(20, 10);
    System.out.println("Matrix: " + m2);
    System.out.println("Rows: " + m2.rows());
    System.out.println("Cols: " + m2.cols());
    m2.show();
    m2.show(false);
    m2.dealloc();
    System.out.println("Matrix: " + m2);
  }
}
