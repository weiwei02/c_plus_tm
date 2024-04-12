package tree

import (
	"testing"
)

func TestDeserialize(t *testing.T) {
	tests := []string{
		"1,2,-1,-1,3,6,-1,-1,7,-1,-1",
	}
	for _, tt := range tests {
		t.Run(tt, func(t *testing.T) {
			got := Deserialize(tt)
			result := Serialize(got)
			if tt != result {
				t.Errorf("intput not equal output %s", result)
			}
		})
	}
}
