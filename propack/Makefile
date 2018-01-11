DOBJ=dlansvd_irl.o

all: clean lib

prepare:
	mkdir -p target

.f.o:
	gfortran -c -fPIC $*.f -o target/$*.o
.F.o: prepare
	gfortran -c -fPIC $*.F -o target/$*.o

lib: prepare $(DOBJ)
	cd target && ar rc libpropack.a $(DOBJ) && cd ..

clean:
	rm -rf target
