package main

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/aws/smithy-go"
)

// main uses the AWS SDK for Go V2 to create an Amazon Simple Storage Service
// (Amazon S3) client and list up to 10 buckets in your account.
// This example uses the default settings specified in your shared credentials
// and config files.
func main() {
	start := time.Now()

	profile := "hakusai"
	bucket := "hakusai-test-bucket"

	ctx := context.Background()
	sdkConfig, err := config.LoadDefaultConfig(ctx, config.WithSharedConfigProfile(profile))
	if err != nil {
		fmt.Println("Couldn't load default configuration. Have you set up your AWS account?")
		fmt.Println(err)
		return
	}
	s3Client := s3.NewFromConfig(sdkConfig)
	configLoaded := time.Since(start)
	fmt.Printf("load config %s\n", configLoaded)
	_, err = s3Client.ListObjectsV2(ctx, &s3.ListObjectsV2Input{Bucket: &bucket})
	if err != nil {
		var ae smithy.APIError
		if errors.As(err, &ae) && ae.ErrorCode() == "AccessDenied" {
			fmt.Println("You don't have permission to list buckets for this account.")
		} else {
			fmt.Printf("Couldn't list buckets for your account. Here's why: %v\n", err)
		}
		return
	}
	requestSent := time.Since(start)
	fmt.Printf("request sent %d\n", requestSent-configLoaded)
}
