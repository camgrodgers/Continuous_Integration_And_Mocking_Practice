pipeline {
	agent any
    stages {
        stage('Check') {
            steps {
                sh 'rustup default stable'
                sh 'cargo check'
            }
        }
        stage('Build') {
            steps {
                sh 'rustup default stable'
                sh 'cargo build'
            }
        }
        stage('UnitTest') {
            steps {
                sh 'cargo test unit'
            }
        }
        stage('DummyTest') {
            steps {
                sh 'cargo test mock'
            }
        }
        stage('HttpTest') {
            steps {
                sh 'cargo test http'
            }
        }
	}
}

