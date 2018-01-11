DOBJ = dlanbpro.o dreorth.o dgetu0.o dsafescal.o dblasext.o \
       dlansvd.o dgemm_ovwr.o \
       dlansvd_irl.o dbsvd.o dritzvec.o dmgs.risc.o second.o

all: clean lib

prepare:
	mkdir -p target

.f.o:
	gfortran -c -fPIC -I. $*.f -o target/$*.o
.F.o:
	gfortran -c -fPIC -I. $*.F -o target/$*.o

lib: prepare $(DOBJ)
	cd target && ar rc libpropack.a $(DOBJ) && cd ..

clean:
	rm -rf target
