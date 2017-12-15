name := "rustjblas"

organization := "com.github.sadikovi"

// scala version for tests
scalaVersion := "2.11.7"

// Compile dependencies
libraryDependencies ++= Seq(
  "org.jblas" % "jblas" % "1.2.4" % "provided"
)

// Test dependencies
libraryDependencies ++= Seq(
  "com.novocode" % "junit-interface" % "0.11" % "test"
)

javacOptions in ThisBuild ++= Seq("-Xlint:unchecked")
scalacOptions in ThisBuild ++= Seq("-unchecked", "-deprecation", "-feature")

// Display full-length stacktraces from ScalaTest
testOptions in Test += Tests.Argument("-oF")
testOptions in Test += Tests.Argument(TestFrameworks.JUnit, "-a", "-v", "+q")

parallelExecution in Test := false
