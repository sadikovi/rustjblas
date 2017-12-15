package com.github.sadikovi.rustjblas;

/**
 * Rustjblas native operation exception, that is thrown when underlying library code panics, or
 * operation cannot be continued.
 *
 * It extends `java.lang.RuntimeException` in order to be an unchecked.
 * Most methods in `com.github.sadikovi.rustjblas.DoubleMatrix` should be expected to throw this
 * type of exception.
 */
public class OperationException extends RuntimeException {
  public OperationException(String message, Throwable cause) {
    super(message, cause);
  }

  public OperationException(String message) {
    super(message);
  }

  public OperationException() {
    super();
  }
}
